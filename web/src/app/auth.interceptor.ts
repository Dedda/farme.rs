import {HttpInterceptorFn} from "@angular/common/http";
import {jwtDecode, JwtPayload} from "jwt-decode";


export const authInterceptor: HttpInterceptorFn = (req, next) => {
   const token = getLoginToken();

    if (token) {
        return next(req.clone({
            headers: req.headers.set('Authorization', token)
        }));
    }
    return next(req);
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