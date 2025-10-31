import {Component} from '@angular/core';
import {Router, RouterLink, RouterOutlet} from '@angular/router';
import {AuthService} from "./auth.service";
import {NgIf} from "@angular/common";
import {FarmService} from "./api/farm.service";
import {UserService} from "./api/user.service";
import {LeafletMapComponent} from "./widget/leaflet-map/leaflet-map.component";

@Component({
    selector: 'app-root',
    imports: [RouterOutlet, RouterLink, NgIf],
    providers: [FarmService, UserService],
    templateUrl: './app.component.html'
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
