import {Component, OnInit} from '@angular/core';
import {AuthService} from "../../auth.service";
import {Router, RouterLink} from "@angular/router";
import {User} from "../../api/models";
import {UserService} from "../../api/user.service";
import {FormsModule, ReactiveFormsModule} from "@angular/forms";

@Component({
    selector: 'app-user-page',
    imports: [
        RouterLink,
        ReactiveFormsModule,
        FormsModule
    ],
    templateUrl: './user-page.component.html',
    styleUrl: './user-page.component.css'
})
export class UserPageComponent implements OnInit {

    user: User = new User('', '', '', '');
    submitting = false;
    password = '';

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

    validate(): boolean {
        return this.password.length >= 8 && this.password.length >= 8;
    }

    delete_action(): void {
        if (!this.validate()) {
            return;
        }
        this.submitting = true;
        this.userService.deleteCurrentUser(this.password).subscribe({
            next: res => {
                if (res) {
                    this.authService.logout();
                    console.log('user deleted');
                    this.router.navigate(['/']);
                }
                this.submitting = false;
            },
            error: err => {
                this.submitting = false;
            }
        });
    }
}
