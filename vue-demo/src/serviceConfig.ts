// serviceConfig.ts
import type { ServiceConfiguration } from './pilotService/serviceConfig';

export const serviceConfigurations: ServiceConfiguration[] = [
  {
    name: 'default',
    type: 'graphql',
    url: 'https://nexus-s2-home-pilotnode-graphql.at.remote.it:8080/graphql',
    wsUrl: 'wss://nexus-s2-home-pilotnode-graphql.at.remote.it:8080/graphql',
  },
];
