import { Injectable } from '@angular/core';
import {HttpClient} from "@angular/common/http";
import {NewUser, User} from "./api/models";
import {Observable} from "rxjs";
import {map} from "rxjs/operators";

@Injectable({
  providedIn: 'root'
})
export class AuthService {

    private baseUrl = 'http://localhost:8000/api/v1';

    constructor(
      private http: HttpClient,
    ) { }

    public register(user: NewUser): Observable<User> {
        return this.http.post<User>(this.baseUrl + '/users/create', user);
    }
}
