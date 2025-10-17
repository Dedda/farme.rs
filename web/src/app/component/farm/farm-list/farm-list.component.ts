import {Component, OnInit} from '@angular/core';
import {Farm, User} from '../../../api/models';
import {Router, RouterLink} from '@angular/router';
import {FarmService} from "../../../api/farm.service";
import {UserService} from "../../../api/user.service";

@Component({
  selector: 'app-farm-list',
  imports: [
    RouterLink
  ],
  providers: [FarmService],
  templateUrl: './farm-list.component.html',
  styleUrl: './farm-list.component.css'
})
export class FarmListComponent implements OnInit {
  user: User | null = null;
  farms: Farm[] = [];

  constructor(private farmService: FarmService, private userService: UserService, private router: Router) {
    this.router.events.subscribe(e => {
      console.log('Router event:', e);
    });
  }

  ngOnInit() {
    this.userService.getCurrentUser().subscribe(user => {
      this.user = user;
    })
    this.farmService.getAll().subscribe(farms => {
      this.farms = farms;
    })
  }
}
