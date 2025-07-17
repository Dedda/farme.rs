import {Component, OnInit} from '@angular/core';
import {AuthService} from "../../auth.service";
import {Router} from "@angular/router";
import {FormsModule} from "@angular/forms";
import {NewUser} from "../../api/models";

@Component({
  selector: 'app-register-page',
    imports: [
        FormsModule
    ],
  templateUrl: './register-page.component.html',
  styleUrl: './register-page.component.css'
})
export class RegisterPageComponent implements OnInit {

    firstname: string = '';
    lastname: string = '';
    username: string = '';
    email: string = '';
    password: string = '';
    confirm_password: string = '';

    submitting: boolean = false;

    constructor(private authService: AuthService, private router: Router) {}

    ngOnInit(): void {

    }

    validate(): boolean {
        if (this.firstname.trim().length < 2) {
            return false;
        }
        if (this.lastname.trim().length < 2) {
            return false;
        }
        if (this.username.trim().length < 3) {
            return false;
        }
        // TODO: validate email
        return this.password.length >= 8 && this.password === this.confirm_password;
    }

    registerAction() {
        if (!this.validate()) {
            return;
        }
        var newUser = new NewUser(this.firstname, this.lastname, this.username, this.email, this.password);
        this.authService.register(newUser).subscribe(res => {
            console.log('user registered: ', res);
            this.router.navigate(['/']).then(r => {});
        });
    }
}
