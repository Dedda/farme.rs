import {Component} from '@angular/core';
import {ActivatedRoute, Router} from '@angular/router';
import {FarmService} from "../../../api/farm.service";
import {Farm} from "../../../api/models";

@Component({
  selector: 'app-farm-details',
  imports: [],
  templateUrl: './farm-details.component.html',
  styleUrl: './farm-details.component.css',
  providers: [FarmService]
})
export class FarmDetailsComponent {

  farm: Farm = new Farm(0, '');
  submitting = false;

  constructor(private farmService: FarmService, private router: Router, route: ActivatedRoute) {
    farmService.getFull(route.snapshot.params['id']).subscribe(farm => {
      this.farm = farm;
    })
  }

  deleteAction(): void {
    if (this.submitting) {
      return;
    }
    this.submitting = true;
    this.farmService.delete(this.farm.id).subscribe(res => {
      if (res) {
        this.router.navigate(['/farms']);
      }
    })
  }
}
