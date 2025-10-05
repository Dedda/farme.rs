import {Component} from '@angular/core';
import {Farm} from '../../../api/models';
import {RouterLink} from '@angular/router';
import {FarmService} from "../../../api/farm.service";

@Component({
  selector: 'app-farm-list',
  imports: [
    RouterLink
  ],
  providers: [FarmService],
  templateUrl: './farm-list.component.html',
  styleUrl: './farm-list.component.css'
})
export class FarmListComponent {
  farms: Farm[] = [];

  constructor(farmService: FarmService) {
    farmService.getAll().subscribe(farms => {
      this.farms = farms;
    })
  }
}
