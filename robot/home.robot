*** Settings ***
Resource        keywords.robot

*** Test Cases ***
Show Index Page
    keywords.Open Browser To Index Page
    Title Should Be     Farmers
    Wait Until Element Is Visible   xpath=//a[text() = "Login"]
    Close Browser

Navigate To Login Page
    keywords.Open Browser To Index Page
    Title Should Be     Farmers
    Wait Until Element Is Visible   xpath=//a[text() = "Login"]
    Click Element                   xpath=//a[text() = "Login"]
    Wait Until Element Is Visible   xpath=//h1[text() = "Login"]
    Close Browser
