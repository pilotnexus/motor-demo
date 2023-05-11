// Service.ts

import type { SubValue } from "./subValue";

export interface Service {
    // Define the common methods for the services
    get(path: string, options?: any): Promise<any>;
    set(path: string, value: any, options?: any): Promise<void>;
    subscribe(path: string, callback: (value: any) => void, subValue?: SubValue): () => void;
}
