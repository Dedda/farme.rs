import {Component, OnInit} from '@angular/core';
import {Router} from "@angular/router";
import {ApiService, PasswordChangeRequest} from "../../api/api.service";
import {AuthService} from "../../auth.service";
import {FormsModule, ReactiveFormsModule} from "@angular/forms";

@Component({
    selector: 'app-password-change-page',
    imports: [
        ReactiveFormsModule,
        FormsModule
    ],
    templateUrl: './password-change-page.component.html',
    styleUrl: './password-change-page.component.css'
})
export class PasswordChangePageComponent implements OnInit {

    change: PasswordChangeRequest = new PasswordChangeRequest('', '');

    submitting = false;

    constructor(private authService: AuthService, private apiService: ApiService, private router: Router) {
    }

    ngOnInit(): void {
        if (!this.authService.isLoggedIn) {
            this.router.navigate(['/login']);
        }
    }

    validate(): boolean {
        return this.change.old_password.length >= 8 && this.change.new_password.length >= 8;
    }

    changeAction() {
        this.submitting = true;
        if (!this.validate()) {
            this.submitting = false;
            return;
        }
        this.apiService.changePassword(this.change).subscribe(_res => {
            this.submitting = false;
            this.router.navigate(['/user']);
        })
    }
}
