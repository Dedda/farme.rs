import {Component} from '@angular/core';
import {RouterOutlet} from '@angular/router';
import {ApiService} from "./api/api.service";

@Component({
  selector: 'app-root',
  imports: [RouterOutlet],
  providers: [ApiService],
  templateUrl: './app.component.html',
  styleUrl: './app.component.css'
})
export class AppComponent {

  title = 'farmers';

}
