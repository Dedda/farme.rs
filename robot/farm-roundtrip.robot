*** Settings ***
Resource    keywords.robot

*** Variables ***
${FIRST NAME}       Robot
${LAST NAME}        Tester
${USERNAME}         robottester
${EMAIL}            robot@test.com
${PASSWORD}         Test!user1
${FARM NAME}        Robotfarm

*** Test Cases ***

Register User
    keywords.Open Browser To Index Page
    keywords.Navigate To Register Page
    Input Text      id:firstname            ${FIRST NAME}
    Input Text      id:lastname             ${LAST NAME}
    Input Text      id:username             ${USERNAME}
    Input Text      id:email                ${EMAIL}
    Input Password  id:password             ${PASSWORD}
    Input Password  id:confirm-password     ${PASSWORD}
    Click Element   id:register-btn
    Wait Until Element Is Visible           xpath=//h1[text() = "Hello, farmers"]

Login Created User
    keywords.Navigate To Login Page
    Input Text      id:identity             ${USERNAME}
    Input Password  id:password             ${PASSWORD}
    Click Element   id:login-btn
    Wait Until Element Is Visible           xpath=//a[text() = "Logout"]

Request Farm Admin Status
    keywords.Navigate To User Profile
    Click Element                           id:request-adm-btn
    Wait Until Element Is Visible           xpath=//p[text() = "Farm Admin"]

Show Farms
    keywords.Navigate To Farm List
    Wait Until Element Is Visible           xpath=//h1[contains(text(), "farm-list")]

Create New Farm
    Wait Until Element Is Visible           xpath=//a[text() = "Create farm"]
    Click Element                           xpath=//a[text() = "Create farm"]
    Wait Until Element Is Visible           xpath=//h1[text() = "Create Farm"]
    Input Text      id:farmname             ${FARM NAME}
    Click Element                           xpath=//button[text() = "Create"]
    Wait Until Element Is Visible           xpath=//h1[text() = "${FARM NAME}"]

Delete Farm
    Wait Until Element Is Visible           id:delete-btn
    Click Element                           id:delete-btn
    Wait Until Element Is Visible           xpath=//h1[contains(text(), "farm-list")]

Delete Changed User
    keywords.Navigate To User Profile
    Wait Until Element Is Visible           xpath=//button[text() = "Delete Account"]
    Input Password  id:password             ${PASSWORD}
    Click Element                           xpath=//button[text() = "Delete Account"]
    Wait Until Element Is Visible           xpath=//a[text() = "Login"]
    Close Browser
