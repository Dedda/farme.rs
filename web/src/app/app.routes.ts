import {Routes} from '@angular/router';
import {FarmListComponent} from './component/farm-list/farm-list.component';
import {FarmDetailsComponent} from './component/farm-details/farm-details.component';
import {HomePageComponent} from './component/home-page/home-page.component';
import {LoginPageComponent} from './component/login-page/login-page.component';
import {RegisterPageComponent} from "./component/register-page/register-page.component";
import {UserPageComponent} from "./component/user-page/user-page.component";
import {UserChangePageComponent} from "./component/user-change-page/user-change-page.component";
import {PasswordChangePageComponent} from "./component/password-change-page/password-change-page.component";

export const routes: Routes = [
    {path: '', component: HomePageComponent, data: {breadcrumb: 'Home'}},
    {path: 'register', component: RegisterPageComponent, data: {breadcrumb: 'Register'}},
    {path: 'login', component: LoginPageComponent, data: {breadcrumb: 'Login'}},
    {path: 'user', component: UserPageComponent, data: {breadcrumb: 'Profile'}},
    {path: 'user/change', component: UserChangePageComponent, data: {breadcrumb: 'Change'}},
    {path: 'user/pwchange', component: PasswordChangePageComponent, data: {breadcrumb: 'Change Password'}},
    {path: 'farms/:id', component: FarmDetailsComponent, data: {breadcrumb: ''}},
    {path: 'farms', component: FarmListComponent, data: {breadcrumb: 'Farms'}},
];
