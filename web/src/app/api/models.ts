export class Farm {
    constructor(public id: number, public name: string) {}
}

export class ShopType {
    constructor(public id: number, public name: string) {}
}

export class OpeningHours {
    constructor(public id: number, public weekday: number, public open: number, public close: number) {}
}

export class FullFarm {
    constructor(
        public id: number,
        public name: string,
        public lat: number,
        public lon: number,
        public shopTypes: ShopType[],
        public openingHours: OpeningHours[]) {}
}

export class NewUser {
    constructor(
        public firstname: string,
       public lastname: string,
       public username: string,
       public email: string,
       public password: string,
    ) {}
}

export class User {
    constructor(
        public firstname: string,
        public lastname: string,
        public username: string,
        public email: string,
        public farmowner: boolean = false,
    ) {}
}

export class NewFarm {
    constructor(
        public name: string,
        public lat: number,
        public lon: number,
    ) {}
}