Feature: the agent retries a request

Scenario: the test fails
Given the test fails
When the agent retries the request
Then the queue is empty
And the Linter shows an error

Scenario: "retry-limit"
Given the agent retries the request
When the test fails
Then the agent does not retry the request
