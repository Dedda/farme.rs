import {Component, OnInit} from '@angular/core';
import {Router, RouterLink, RouterOutlet} from '@angular/router';
import {ApiService} from "./api/api.service";
import {AuthService} from "./auth.service";
import {NgIf} from "@angular/common";
import {Breadcrumb, BreadcrumbService} from "./breadcrumb.service";

@Component({
    selector: 'app-root',
    imports: [RouterOutlet, RouterLink, NgIf],
    providers: [ApiService],
    templateUrl: './app.component.html',
    styleUrl: './app.component.css'
})
export class AppComponent implements OnInit {

    title = 'farmers';
    breadcrumbs: Array<Breadcrumb> = [];

    constructor(private authService: AuthService, private router: Router, private breadcrumbService: BreadcrumbService) {}

    ngOnInit(): void {
        this.breadcrumbs = this.breadcrumbService.breadcrumbs;
    }

    isLoggedIn(): boolean {
        return this.authService.isLoggedIn();
    }

    logout() {
        this.authService.logout();
        this.router.navigate(['/']);
    }
}
