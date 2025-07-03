import { Routes } from '@angular/router';
import { FarmListComponent } from './component/farm-list/farm-list.component';
import { FarmDetailsComponent } from './component/farm-details/farm-details.component';
import { HomePageComponent } from './component/home-page/home-page.component';
import {LoginPageComponent} from './component/login-page/login-page.component';

export const routes: Routes = [
  { path: '', component: HomePageComponent },
  { path: 'login', component: LoginPageComponent },
  { path: 'farms/:id', component: FarmDetailsComponent },
  { path: 'farms', component: FarmListComponent },
];
