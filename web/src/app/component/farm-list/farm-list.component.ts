import {Component} from '@angular/core';
import {Farm} from '../../api/models';
import {ApiService} from '../../api/api.service';
import {RouterLink} from '@angular/router';

@Component({
  selector: 'app-farm-list',
  imports: [
    RouterLink
  ],
  providers: [ApiService],
  templateUrl: './farm-list.component.html',
  styleUrl: './farm-list.component.css'
})
export class FarmListComponent {
  farms: Farm[] = [];

  constructor(apiService: ApiService) {
    apiService.getAllFarms().subscribe(farms => {
      this.farms = farms;
    })
  }
}
