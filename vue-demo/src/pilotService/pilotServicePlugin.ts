// pilotServicePlugin.ts
import type { App } from 'vue';
import type { Service } from './Service';
import type { ServiceConfiguration } from './serviceConfig';
import { createSetDirective } from './setDirective';
import { GraphQLService } from './services/graphql/graphqlService';
//import { DeepstreamService } from './services/deepstream/DeepstreamService';

export class PilotServicePlugin {
    private services: Record<string, Service> = {};

    constructor(serviceConfigs: ServiceConfiguration[]) {
        serviceConfigs.forEach((config: ServiceConfiguration) => {
            let serviceInstance: Service | null = null;

            if (config.type === 'deepstream') {
                //serviceInstance = new DeepstreamService(config.url as string);
            } else if (config.type === 'graphql') {
                serviceInstance = new GraphQLService(config.url, config.wsUrl);
            }

            if (serviceInstance) {
                this.services[config.name] = serviceInstance;
            }
        });
    }

    install(app: App): void {
        app.directive('set', createSetDirective(this));
        app.config.globalProperties.$set = createSetFunction(this);

        app.provide('pilotService', this);

        const localVariables: Record<string, any> = {};
        app.provide('localVariables', localVariables);
    }

    getValue(variable: string, serviceName?: string): any {
        const service = this.getService(serviceName);
        return service.get(variable);
    }

    setValue(variable: string, value: any, serviceName?: string): void {
        const service = this.getService(serviceName);
        service.set(variable, value);
    }

    subscribe(variable: string, callback: (value: any) => void, serviceName?: string): () => void {
        // console.log(`subscribe called for ${variable}`);
        const service = this.getService(serviceName);
        return service.subscribe(variable, callback);
    }

    private getService(serviceName?: string): Service {
        if (!serviceName || !this.services[serviceName]) {
            if (!this.services['default'])
                throw Error("No default service defined");
            return this.services['default'];
        }

        return this.services[serviceName];
    }

}

export function createSetFunction(pilotServicePlugin: PilotServicePlugin) {
    const set = (path: string, value: any) => {
        pilotServicePlugin.setValue(path, value);
    };

    return set;
}
