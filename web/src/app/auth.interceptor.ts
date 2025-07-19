import {HttpInterceptorFn, HttpRequest, HttpResponse} from "@angular/common/http";
import {jwtDecode, JwtPayload} from "jwt-decode";
import {tap} from "rxjs";


export const authInterceptor: HttpInterceptorFn = (req, next) => {
   const token = getLoginToken();

   var newRequest = req;

    if (token) {
        newRequest = req.clone({
            headers: req.headers.set('Authorization', token)
        });
    }
    return next(newRequest).pipe(tap({
        next: event => {
            if (event instanceof HttpResponse) {
                console.log('response');
                let headers = event.headers;
                // console.log(headers);
                let auth = headers.get('Authorization');
                if (auth) {
                    console.log('Updating login token');
                    localStorage.setItem('token', auth);
                }
            }
        }
    }));
}

export function getLoginToken(): string | null {
    const token = localStorage.getItem('token');
    if (token) {
        const decoded: JwtPayload = jwtDecode(token);
        if (!decoded.exp) {
            localStorage.removeItem('token');
            return null;
        }
        const expiration = new Date(decoded.exp * 1000);
        const now = new Date();
        if (expiration.getTime() < now.getTime()) {
            console.log('login token timed out');
            localStorage.removeItem('token');
            return null;
        }
    }
    return token;
}