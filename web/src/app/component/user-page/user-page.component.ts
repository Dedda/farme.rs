import {Component, OnInit} from '@angular/core';
import {AuthService} from "../../auth.service";
import {Router, RouterLink} from "@angular/router";
import {User} from "../../api/models";
import {UserService} from "../../api/user.service";

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
        private userService: UserService) {
    }

    ngOnInit(): void {
        if (!this.authService.isLoggedIn()) {
            void this.router.navigate(['/login']);
        } else {
            this.userService.getCurrentUser().subscribe(user => {
                console.log(user);
                this.user = user;
            });
        }
    }
}
