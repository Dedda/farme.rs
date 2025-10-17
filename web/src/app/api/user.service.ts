import { Injectable } from '@angular/core';
import {HttpClient, HttpHeaders} from "@angular/common/http";
import {Observable} from "rxjs";
import {NewUser, User} from "./models";
import {Api} from "./api";
import {map} from "rxjs/operators";

@Injectable({
  providedIn: 'root'
})
export class UserService {

  constructor(
      private http: HttpClient,
  ) { }

  public getCurrentUser(): Observable<User> {
    const h = new HttpHeaders({ 'Content-Type': 'application/json' });
    return this.http.get<User>(Api.API_BASE_URL + '/users/current-user', {headers: h});
  }

  public updateCurrentUser(user: NewUser): Observable<boolean> {
    const h = new HttpHeaders({ 'Content-Type': 'application/json' });
    return this.http.post(Api.API_BASE_URL + '/users/change', user, {headers: h}).pipe(map(_res => true));
  }

  public changePassword(change: PasswordChangeRequest): Observable<boolean> {
    const h = new HttpHeaders({ 'Content-Type': 'application/json' });
    return this.http.post(Api.API_BASE_URL + '/users/change-password', change, {headers: h}).pipe(map(_res => true));
  }
}

export class PasswordChangeRequest {
  constructor(public old_password: string, public new_password: string) {}
}