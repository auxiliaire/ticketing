Feature: Application Feature

    Scenario: If we run the main function, the application will start
        Given the main function has been called
        When I send a request
        Then a response will be received