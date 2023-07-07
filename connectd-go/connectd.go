package main

import (
	"fmt"
	"os"

	"github.com/SUSE/connect-ng/internal/connect"
	"github.com/godbus/dbus/v5"
	"github.com/godbus/dbus/v5/introspect"
)

// maybe we we can avoid XML with this: https://github.com/godbus/dbus/blob/master/_examples/prop.go
const intro = `
<node>
	<interface name="com.github.suse.ConnectdGo">
		<method name="Version">
			<arg direction="in" type="b"/>
			<arg direction="out" type="s"/>
		</method>
	</interface>` + introspect.IntrospectDataString + `</node> `

type foo string

func (f foo) Version(fullVersion bool) (string, *dbus.Error) {
	var version string
	if fullVersion {
		version = connect.GetFullVersion()
	} else {
		version = connect.GetShortenedVersion()
	}

	return version, nil
}

func main() {
	conn, err := dbus.ConnectSessionBus()
	if err != nil {
		panic(err)
	}
	defer conn.Close()

	f := foo("Quack!")
	conn.Export(f, "/com/github/suse/ConnectdGo", "com.github.suse.ConnectdGo")
	conn.Export(introspect.Introspectable(intro), "/com/github/suse/ConnectdGo",
		"org.freedesktop.DBus.Introspectable")

	reply, err := conn.RequestName("com.github.suse.ConnectdGo",
		dbus.NameFlagDoNotQueue)
	if err != nil {
		panic(err)
	}
	if reply != dbus.RequestNameReplyPrimaryOwner {
		fmt.Fprintln(os.Stderr, "name already taken")
		os.Exit(1)
	}
	fmt.Println("Listening on com.github.suse.ConnectdGo / /com/github/suse/ConnectdGo ...")
	select {}
}
