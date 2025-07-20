import {Component, OnInit} from '@angular/core';
import {NewUser, User} from "../../api/models";
import {AuthService} from "../../auth.service";
import {ApiService} from "../../api/api.service";
import {Router} from "@angular/router";
import {FormsModule, ReactiveFormsModule} from "@angular/forms";

@Component({
  selector: 'app-user-change-page',
    imports: [
        ReactiveFormsModule,
        FormsModule
    ],
  templateUrl: './user-change-page.component.html',
  styleUrl: './user-change-page.component.css'
})
export class UserChangePageComponent implements OnInit {

    user: NewUser = new NewUser('', '', '', '', '');
    submitting: boolean = false;

    constructor(private authService: AuthService, private apiService: ApiService, private router: Router) {
    }

    ngOnInit(): void {
        if (!this.authService.isLoggedIn()) {
            this.router.navigate(['/login']);
        } else {
            this.apiService.getCurrentUser().subscribe(user => {
                console.log(user);
                this.user = new NewUser(
                    user.firstname,
                    user.lastname,
                    user.username,
                    user.email,
                    ''
                );
            });
        }
    }

    validate(): boolean {
        if (this.user.firstname.trim().length < 2) {
            return false;
        }
        if (this.user.lastname.trim().length < 2) {
            return false;
        }
        if (this.user.username.trim().length < 3) {
            return false;
        }
        // TODO: validate email
        return this.user.password.length >= 8;
    }

    changeAction() {
        this.submitting = true;
        if (!this.validate()) {
            this.submitting = false;
            return;
        }
        this.apiService.updateCurrentUser(this.user).subscribe(user => {
            this.submitting = false;
            this.router.navigate(['/user']);
        })
    }
}
