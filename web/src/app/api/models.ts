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