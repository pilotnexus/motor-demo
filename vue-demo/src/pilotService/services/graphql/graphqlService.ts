// GraphQLService.ts
import type { Service } from './../../Service';
import { SubValue } from './../../subValue';

import { ApolloClient, InMemoryCache, split, HttpLink, gql, type DocumentNode } from '@apollo/client';
import { getMainDefinition } from '@apollo/client/utilities';

import { GraphQLWsLink } from '@apollo/client/link/subscriptions';
import { createClient } from "graphql-ws";

//import { WebSocketLink } from "@apollo/client/link/ws";
//import { SubscriptionClient } from "subscriptions-transport-ws";


interface SubscriptionData {
    subscribers: Set<(value: any) => void>;
    unsubscribe: () => void;
}

export class GraphQLService implements Service {
    private client: ApolloClient<any>;
    private subscriptionMap: Map<string, SubscriptionData> = new Map();

    constructor(httpUrl: string, wsUrl: string) {
        // Initialize HTTP link
        const httpLink = new HttpLink({
            uri: httpUrl,
        }); `~`

        // Replace with this after
        // Initialize WebSocket link
        const wsLink = new GraphQLWsLink(createClient({
            url: wsUrl,
        }));

        // Initialize WebSocket link
        // const wsLink = new WebSocketLink(
        //   new SubscriptionClient(wsUrl, {
        //     reconnect: true
        //   })
        // );

        // Split communication by operation (recommended)
        const splitLink = split(
            ({ query }: { query: DocumentNode }) => {
                const definition = getMainDefinition(query);
                return (
                    definition.kind === 'OperationDefinition' &&
                    definition.operation === 'subscription'
                );
            },
            wsLink,
            httpLink,
        );

        // Initialize Apollo Client
        this.client = new ApolloClient({
            link: splitLink,
            cache: new InMemoryCache(),
        });

    }

    async get(path: string, options?: any, subValue: SubValue = SubValue.ActualValue): Promise<any> {
        const query = gql`
      query {
        values {
          ${path.replace('.', ' { ')} {
            ${subValue}
          }${path.split('.').map(() => '}').join('')}
        }
      }
    `;

        const result = await this.client.query({ query });
        return result.data.values[path.split('.')[0]][path.split('.')[1]][subValue];
    }

    async set(path: string, value: any, options?: any, subValue: SubValue = SubValue.TargetValue): Promise<void> {
        const mutation = gql`
      mutation {
        setValue(name: "${path}", value: "${value}", subValue: "${subValue}")
      }
    `;
        await this.client.mutate({ mutation });
    }

    subscribe(
        path: string,
        callback: (value: any) => void,
        subValue: SubValue = SubValue.ActualValue
    ): () => void {
        const key = `${path}.${subValue}`;
        let subscriptionData = this.subscriptionMap.get(key);

        if (!subscriptionData) {
            const queryPath = path
                .split('.')
                .map((part) => `${part} {`)
                .join('') + `${subValue}` + '}'.repeat(path.split('.').length);

            const subscription = gql`
        subscription {
          subscribeValue(name: "${path}", subValue: "${subValue}")
        }
      `;

            const observable = this.client.subscribe({ query: subscription });
            const subscriptionHandler = observable.subscribe({
                next: ({ data }) => {
                    const subscribers = this.subscriptionMap.get(key)?.subscribers;
                    const value = data.subscribeValue;
                    subscribers?.forEach((subscriber) => subscriber(value));
                },
                error: (error) => {
                    console.error('Subscription error:', error); // Add this line
                },
            });

            subscriptionData = {
                subscribers: new Set(),
                unsubscribe: () => subscriptionHandler.unsubscribe(),
            };

            this.subscriptionMap.set(key, subscriptionData);
        }

        subscriptionData.subscribers.add(callback);

        return () => {
            subscriptionData?.subscribers.delete(callback);
            // console.log(`Unsubscribe for {key}, remaining subscriptions for that key: ${subscriptionData?.subscribers.size}`);
            if (subscriptionData?.subscribers.size === 0) {
                // console.log("Removing GraphQL subscription");
                subscriptionData?.unsubscribe();
                this.subscriptionMap.delete(key);
            }
        }
    }
}
