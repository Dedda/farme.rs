import { Component } from '@angular/core';
import {LeafletMapComponent} from "../../../widget/leaflet-map/leaflet-map.component";

@Component({
  selector: 'app-farm-map',
  imports: [
    LeafletMapComponent
  ],
  templateUrl: './farm-map.component.html',
  styleUrl: './farm-map.component.scss'
})
export class FarmMapComponent {

}
