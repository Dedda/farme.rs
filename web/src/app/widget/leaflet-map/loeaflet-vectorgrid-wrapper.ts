import * as L from 'leaflet';
import 'leaflet.vectorgrid';

// Patch vectorGrid onto L if missing
// This fixes the problem that during optimization, leaflet might be renamed from L and vectorgrid might not work
const leafletWithVectorGrid = {
    ...L,
    vectorGrid: (L as any).vectorGrid || (window as any).L?.vectorGrid
};

export default leafletWithVectorGrid;