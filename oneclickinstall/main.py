#!/usr/bin/python
import xml.dom.minidom
import subprocess
import re
from gi.repository import Gtk
from gi.repository import PackageKitGlib as packagekit


document = """\
<?xml version="1.0" encoding="utf-8"?>
<metapackage xmlns:os="http://opensuse.org/Standards/One_Click_Install" xmlns="http://opensuse.org/Standards/One_Click_Install">
  <group distversion="openSUSE 11.4">
    <repositories>
      <repository recommended="true">
        <name>utilities</name>
        <url>http://download.opensuse.org/repositories/utilities/openSUSE_11.4/</url>
      </repository>
    </repositories>
    <software>
      <item>
        <name>atool</name>
      </item>
    </software>
  </group>
  <group distversion="openSUSE 12.1">
    <repositories>
      <repository recommended="true">
        <name>utilities</name>
        <url>http://download.opensuse.org/repositories/utilities/openSUSE_12.1/</url>
      </repository>
    </repositories>
    <software>
      <item>
        <name>atool</name>
      </item>
    </software>
  </group>
</metapackage>
"""


def progress_cb(status, typ, data=None):
    if status.get_property('package'):
        print "Pachet ", status.get_property('package'), status.get_property('package-id')
        if status.get_property('package'):
            print status.get_property('package').get_name()
    print typ, status.get_property('package')

class MyWindow(Gtk.Window):

    def __init__(self):
        Gtk.Window.__init__(self, title="Hello World")

        self.button = Gtk.Button(label="Click Here")
        self.button.connect("clicked", self.on_button_clicked)
        self.add(self.button)

    def on_button_clicked(self, widget):
        print "Hello World"

win = MyWindow()
#win.connect("delete-event", Gtk.main_quit)
#win.show_all()
#Gtk.main()

def getText(nodelist):
    rc = []
    for node in nodelist:
        if node.nodeType == node.TEXT_NODE:
            rc.append(node.data)
    return ''.join(rc)

#getText(point.childNodes)

distversion = subprocess.Popen("lsb_release -d", stdout=subprocess.PIPE, shell=True).stdout.read()

dom = xml.dom.minidom.parseString(document)
groups = dom.getElementsByTagName("group")
for group in groups:
    group_dist = group.getAttribute("distversion")
    if group_dist in distversion:
        print "MATCH!! %s" % group_dist
    else:
        print distversion
        print group_dist

exit(1)

client = packagekit.Client()
result = client.search_names(0, ["rpm",], None, progress_cb, None)
pkgs = result.get_package_array()
for p in pkgs:
    print p.get_name()


