import { Component } from '@angular/core';
import {FormsModule} from "@angular/forms";
import {FarmService} from "../../../api/farm.service";
import {Router} from "@angular/router";
import {NewFarm} from "../../../api/models";

@Component({
  selector: 'app-create-farm',
    imports: [
        FormsModule
    ],
  templateUrl: './create-farm.component.html',
  styleUrl: './create-farm.component.css'
})
export class CreateFarmComponent {

  farmname: string = '';
  location_override: boolean = false;
  lat: number = 0;
  lon: number = 0;

  submitting: boolean = false;

  constructor(private farmService: FarmService, private router: Router) {}

  validate(): boolean {
    return this.farmname.trim().length > 2;
  }

  createAction() {
    this.submitting = true;
    if (!this.validate()) {
      this.submitting = false;
      return;
    }
    let newFarm = new NewFarm(this.farmname, 0.0, 0.0);
    if (this.location_override) {
      newFarm.lat = this.lat;
      newFarm.lon = this.lon;
    }
    this.farmService.create(newFarm).subscribe(res => {
      console.log('Farm created: ', res);
      this.router.navigate(['/farms', '' + res.id]).then(_ => {});
      this.submitting = false;
    });
  }
}
