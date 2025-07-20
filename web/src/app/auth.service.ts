import {Injectable} from '@angular/core';
import {HttpClient} from "@angular/common/http";
import {NewUser, User} from "./api/models";
import {Observable} from "rxjs";
import {map} from "rxjs/operators";
import {getLoginToken} from "./auth.interceptor";

@Injectable({
  providedIn: 'root'
})
export class AuthService {

    private baseUrl = 'http://localhost:8000';
    private apiBaseUrl = this.baseUrl + '/api/v1';

    constructor(
      private http: HttpClient,
    ) { }

    public register(user: NewUser): Observable<User> {
        return this.http.post<User>(this.apiBaseUrl + '/users/create', user);
    }

    public login(credentials: LoginCredentials): Observable<boolean> {
        return this.http.post(this.apiBaseUrl + '/ident/login-jwt', credentials, {responseType: "text"}).pipe(map(res => {
            this.setSession(res);
            return true;
        }));
    }

    public logout() {
        localStorage.removeItem('token');
    }

    public isLoggedIn(): boolean {
        return getLoginToken() != null;
    }

    private setSession(token: string) {
        localStorage.setItem('token', token);
    }
 }

export class LoginCredentials {
    constructor(
        public identity: string,
        public password: string,
    ) { }
}