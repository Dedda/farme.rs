import {AfterViewInit, Component, OnInit} from '@angular/core';
import L from './loeaflet-vectorgrid-wrapper';
import {HttpClient} from "@angular/common/http";
import { vectorTileStyling } from './vector-tiles-style';

@Component({
  selector: 'app-leaflet-map',
  imports: [],
  templateUrl: './leaflet-map.component.html',
})
export class LeafletMapComponent implements OnInit, AfterViewInit {

  private map!: L.Map;
  markers: L.Marker[] = [
    L.marker([47.0559317,8.2909128]) // Dhaka, Bangladesh
  ];

  constructor(private http: HttpClient) { }

  ngOnInit() {

  }

  private initMap() {
    // const baseMapURl = 'http://localhost:3000/switzerland/{z}/{x}/{y}'
    this.map = L.map('map');

    const vectorTileOptions = {
      // rendererFactory: L.canvas.tile,
      vectorTileLayerStyles: vectorTileStyling,
      maxZoom: 20,
      maxNativeZoom: 15
    };

    console.log('vec: ', vectorTileOptions);

    L.vectorGrid
        .protobuf('http://localhost:3000/tiles/{z}/{x}/{y}', vectorTileOptions)
        .addTo(this.map);
    this.map.eachLayer(layer => {
      console.log(layer);
    });
  }

  private centerMap() {
    // Create a boundary based on the markers
    // const bounds = L.latLngBounds(this.markers.map(marker => marker.getLatLng()));
    // const bounds = L.latLngBounds(this.markers.map(marker => marker.getLatLng()));

    // Fit the map into the boundary
    this.map.fitBounds(L.latLngBounds(L.latLng(47.0559317,8.2909128), L.latLng(47.0286126,8.3416718)));
  }

  ngAfterViewInit() {
    this.initMap();
    this.centerMap();
  }
}
