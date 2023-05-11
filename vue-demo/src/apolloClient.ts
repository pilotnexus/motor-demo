import { gql } from "@apollo/client";

export const SUBSCRIBE_VALUE = gql`
subscription subscribeValue($value: String!, $subValue: String = "actualValue") {
  subscribeValue(name: $value, subValue: $subValue) {
    name
    value
  }
}
`;
