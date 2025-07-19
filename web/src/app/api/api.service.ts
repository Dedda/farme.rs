import {Injectable} from "@angular/core";
import {HttpClient, HttpHeaders} from "@angular/common/http";
import {Farm, FullFarm, NewUser, User} from "./models";
import { map } from 'rxjs/operators';
import {Observable} from 'rxjs';
import {AuthService} from "../auth.service";

@Injectable()
export class ApiService {

    private baseUrl = 'http://localhost:8000/api/v1';

    constructor(private httpClient: HttpClient) { }

    public getAllFarms(): Observable<Farm[]> {
      const h = new HttpHeaders({ 'Content-Type': 'application/json' });
      return this.httpClient.get<Farm[]>(this.baseUrl + '/farms', {headers: h})
        .pipe(map(response => {
            const data: any = response;
            return data;
          }
        ));
    }

    public getFullFarm(id: number): Observable<FullFarm> {
      const h = new HttpHeaders({ 'Content-Type': 'application/json' });
      return this.httpClient.get<FullFarm[]>(this.baseUrl + '/farms/' + id, {headers: h})
        .pipe(map(response => {
            const data: any = response;
            return data;
          }
        ));
    }

    public getCurrentUser(): Observable<User> {
        const h = new HttpHeaders({ 'Content-Type': 'application/json' });
        return this.httpClient.get<User>(this.baseUrl + '/users/current-user', {headers: h});
    }
}
