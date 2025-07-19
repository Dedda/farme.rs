import { Component } from '@angular/core';
import {FormsModule, ReactiveFormsModule} from "@angular/forms";
import {AuthService, LoginCredentials} from "../../auth.service";
import {Router} from "@angular/router";
import {NewUser} from "../../api/models";

@Component({
  selector: 'app-login-page',
    imports: [
        FormsModule,
        ReactiveFormsModule
    ],
  templateUrl: './login-page.component.html',
  styleUrl: './login-page.component.css'
})
export class LoginPageComponent {

    identity: string = '';
    password: string = '';

    submitting = false;

    constructor(private authService: AuthService, private router: Router) {}


    validate(): boolean {
        if (this.identity.trim().length < 3) {
            return false;
        }
        return this.password.length >= 8;
    }

    loginAction() {
        this.submitting = true;
        if (!this.validate()) {
            return;
        }
        var credentials = new LoginCredentials(this.identity, this.password);
        this.authService.login(credentials).subscribe({
            next: res => {
                    if (res) {
                        console.log('user logged in');
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
