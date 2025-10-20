import {AfterViewInit, Component, OnInit} from '@angular/core';
import L from './loeaflet-vectorgrid-wrapper';

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

  constructor() {}
  ngOnInit() {

  }

  private initMap() {
    // const baseMapURl = 'http://localhost:3000/switzerland/{z}/{x}/{y}'
    this.map = L.map('map');

    L.vectorGrid
        .protobuf('http://localhost:3000/tiles/{z}/{x}/{y}', {
        })
        .addTo(this.map);
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
