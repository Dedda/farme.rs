*** Settings ***
Library           SeleniumLibrary

*** Variables ***
${SERVER}       localhost:8000
${BROWSER}      Firefox
${INDEX URL}    http://${SERVER}/

*** Keywords ***
Open Browser To Index Page
    Open Browser    ${INDEX URL}    ${BROWSER}

Navigate To Register Page
    Go To           ${INDEX URL}
    Wait Until Element Is Visible   xpath=//a[text() = "Register"]
    Click Element                   xpath=//a[text() = "Register"]
    Wait Until Element Is Visible   xpath=//h1[starts-with(text(), "Register")]

Navigate To Login Page
    Go To           ${INDEX URL}
    Wait Until Element Is Visible   xpath=//a[text() = "Login"]
    Click Element                   xpath=//a[text() = "Login"]
    Wait Until Element Is Visible   xpath=//h1[starts-with(text(), "Login")]

Navigate To Farm List
    Go To           ${INDEX URL}
    Wait Until Element Is Visible       xpath=//a[text() = "Farm list"]
    Click Element                       xpath=//a[text() = "Farm list"]
    Wait Until Page Contains Element    tag:ul
