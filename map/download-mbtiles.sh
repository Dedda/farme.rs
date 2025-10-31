#!/usr/bin/env bash

# Currently defaults to download only switzerland

docker run\
  --user=$UID\
  -e JAVA_TOOL_OPTIONS="-Xmx32g"\
  -v "$(pwd)/planetiler:/data"\
  --rm\
  ghcr.io/onthegomap/planetiler:latest \
  --download\
  --download-threads=10 --download-chunk-size-mb=1000\
  --fetch-wikidata\
  --minzoom=0\
  --maxzoom=15\
  --tile_compression=none\
  --area=switzerland\
  --output /data/tiles.mbtiles
#  --nodemap-type=array --storage=mmap\

cp -r planetiler/tiles.mbtiles martin/
