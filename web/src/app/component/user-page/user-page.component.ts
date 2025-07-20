import {Component, OnInit} from '@angular/core';
import {AuthService} from "../../auth.service";
import {Router, RouterLink} from "@angular/router";
import {User} from "../../api/models";
import {ApiService} from "../../api/api.service";

@Component({
    selector: 'app-user-page',
    imports: [
        RouterLink
    ],
    templateUrl: './user-page.component.html',
    styleUrl: './user-page.component.css'
})
export class UserPageComponent implements OnInit {

    user: User = new User('', '', '', '');

    constructor(
        private authService: AuthService,
        private router: Router,
        private apiService: ApiService) {
    }

    ngOnInit(): void {
        if (!this.authService.isLoggedIn()) {
            this.router.navigate(['/login']);
        } else {
            this.apiService.getCurrentUser().subscribe(user => {
                console.log(user);
                this.user = user;
            });
        }
    }
}
