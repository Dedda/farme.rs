import {Component} from '@angular/core';
import {Router, RouterLink, RouterOutlet} from '@angular/router';
import {ApiService} from "./api/api.service";
import {AuthService} from "./auth.service";
import {NgIf} from "@angular/common";

@Component({
    selector: 'app-root',
    imports: [RouterOutlet, RouterLink, NgIf],
    providers: [ApiService],
    templateUrl: './app.component.html',
    styleUrl: './app.component.css'
})
export class AppComponent {

    title = 'farmers';

    constructor(private authService: AuthService, private router: Router) {
    }

    isLoggedIn(): boolean {
        return this.authService.isLoggedIn();
    }

    logout() {
        this.authService.logout();
        this.router.navigate(['/']);
    }
}
