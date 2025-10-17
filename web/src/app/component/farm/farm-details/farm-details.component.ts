import {Component} from '@angular/core';
import {ActivatedRoute} from '@angular/router';
import {FarmService} from "../../../api/farm.service";

@Component({
  selector: 'app-farm-details',
  imports: [],
  templateUrl: './farm-details.component.html',
  styleUrl: './farm-details.component.css',
  providers: [FarmService, ActivatedRoute]
})
export class FarmDetailsComponent {

  title = 'Loading farm...';

  constructor(farmService: FarmService, route: ActivatedRoute) {
    farmService.getFull(route.snapshot.params['id']).subscribe(farm => {
      this.title = farm.name;
    })
  }
}
