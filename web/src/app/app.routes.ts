import {Routes} from '@angular/router';
import {FarmListComponent} from './component/farm/farm-list/farm-list.component';
import {FarmDetailsComponent} from './component/farm/farm-details/farm-details.component';
import {HomePageComponent} from './component/home-page/home-page.component';
import {LoginPageComponent} from './component/login-page/login-page.component';
import {RegisterPageComponent} from "./component/register-page/register-page.component";
import {UserPageComponent} from "./component/user/user-page/user-page.component";
import {UserChangePageComponent} from "./component/user/user-change-page/user-change-page.component";
import {PasswordChangePageComponent} from "./component/user/password-change-page/password-change-page.component";
import {CreateFarmComponent} from "./component/farm/create-farm/create-farm.component";

export const routes: Routes = [
    {path: '', component: HomePageComponent},
    {path: 'register', component: RegisterPageComponent},
    {path: 'login', component: LoginPageComponent},
    {path: 'user', component: UserPageComponent},
    {path: 'user/change', component: UserChangePageComponent},
    {path: 'user/pwchange', component: PasswordChangePageComponent},
    {path: 'farms/create', component: CreateFarmComponent},
    {path: 'farms/:id', component: FarmDetailsComponent},
    {path: 'farms', component: FarmListComponent}
];
