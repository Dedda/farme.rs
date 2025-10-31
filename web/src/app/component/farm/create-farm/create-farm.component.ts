import {Component, OnInit} from '@angular/core';
import {FormsModule} from "@angular/forms";
import {FarmService} from "../../../api/farm.service";
import {Router} from "@angular/router";
import {NewFarm} from "../../../api/models";
import {AuthService} from "../../../auth.service";

@Component({
  selector: 'app-create-farm',
    imports: [
        FormsModule
    ],
  templateUrl: './create-farm.component.html'
})
export class CreateFarmComponent implements OnInit {

  farmname: string = '';
  location_override: boolean = false;
  lat: number = 0;
  lon: number = 0;

  submitting: boolean = false;

  constructor(private authService: AuthService, private farmService: FarmService, private router: Router) {}

  ngOnInit() {
    if (!this.authService.isLoggedIn) {
      this.router.navigate(['/login']);
    }
  }

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
    console.log('creating new farm', newFarm);
    this.farmService.create(newFarm).subscribe(res => {
      console.log('Farm created: ', res);
      this.submitting = false;
      this.router.navigate(['/farms', res.id]);
    });
  }
}
