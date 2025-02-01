#!/bin/bash
set -x -e

# MM's doc-version trips up ./gir
xmlstarlet ed -L \
	-d '///_:doc-version' \
	NM-1.0.gir

# NM uses uint32 instead of guint32 in one place:
xmlstarlet ed -L \
	-i '//_:interface[@name="Connection"]/_:method[@name="diff"]//_:parameter[@name="out_settings"]//_:type[@c:type="uint32"]' -t 'attr' -n 'name' -v 'guint32' \
	NM-1.0.gir

# Nuke gcr rather than fixing Gck
xmlstarlet ed -L \
	-d '///_:include[@name="Gcr"]' \
  -d '///_:include[@name="GnomeBluetooth"]' \
	Phosh-0.gir
