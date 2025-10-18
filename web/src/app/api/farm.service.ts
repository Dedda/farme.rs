import { Injectable } from '@angular/core';
import {HttpClient, HttpHeaders} from "@angular/common/http";
import {Farm, FullFarm, NewFarm, NewUser, User} from "./models";
import {Observable} from "rxjs";
import {Api} from "./api";
import {map} from "rxjs/operators";

@Injectable({
  providedIn: 'root'
})
export class FarmService {

  constructor(
      private http: HttpClient,
  ) { }

  public getAll(): Observable<Farm[]> {
    const h = new HttpHeaders({ 'Content-Type': 'application/json' });
    return this.http.get<Farm[]>(Api.API_BASE_URL + '/farms', {headers: h})
        .pipe(map(response => {
              const data: any = response;
              return data;
            }
        ));
  }

  public getFull(id: number): Observable<FullFarm> {
    const h = new HttpHeaders({ 'Content-Type': 'application/json' });
    return this.http.get<FullFarm[]>(Api.API_BASE_URL + '/farms/' + id, {headers: h})
        .pipe(map(response => {
              const data: any = response;
              return data;
            }
        ));
  }

  public create(newFarm: NewFarm): Observable<Farm> {
    return this.http.post<Farm>(Api.API_BASE_URL + '/farms', newFarm);
  }

  public delete(id: number): Observable<boolean> {
      const h = new HttpHeaders({ 'Content-Type': 'application/json' });
      return this.http.delete<boolean>(Api.API_BASE_URL + '/farms/' + id, {headers: h});
  }
}
