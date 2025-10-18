import {Component, OnInit} from '@angular/core';
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
export class FarmDetailsComponent implements OnInit {

  farm: Farm = new Farm(0, '');
  submitting = false;

  constructor(private farmService: FarmService, private route: ActivatedRoute, private router: Router) { }

  ngOnInit(): void {
    this.farmService.getFull(this.route.snapshot.params['id']).subscribe(farm => {
      this.farm = farm;
    })
  }

  deleteAction(): void {
    if (this.submitting) {
      return;
    }
    this.submitting = true;
    this.farmService.delete(this.farm.id).subscribe({
      next: res => {
        this.submitting = false;
        console.log('farm deleted: ' + res);
        this.router.navigate(['/farms']);
      },
      error: err => {
        this.submitting = false;
      }
  });
  }
}
