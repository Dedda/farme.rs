import {Component} from '@angular/core';
import {ApiService} from '../../api/api.service';
import {ActivatedRoute} from '@angular/router';

@Component({
  selector: 'app-farm-details',
  imports: [],
  templateUrl: './farm-details.component.html',
  styleUrl: './farm-details.component.css',
  providers: [ApiService, ActivatedRoute]
})
export class FarmDetailsComponent {

  title = 'Loading farm...';

  constructor(apiService: ApiService, route: ActivatedRoute) {
    apiService.getFullFarm(route.snapshot.params['id']).subscribe(farm => {
      this.title = farm.name;
    })
  }
}
