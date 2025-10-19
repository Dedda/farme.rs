*** Settings ***
Resource    keywords.robot

*** Variables ***
${FIRST NAME}       Robot
${LAST NAME}        Tester
${USERNAME}         robottester
${EMAIL}            robot@test.com
${PASSWORD}         Test!user1

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

Show Profile
    keywords.Navigate To User Profile
    Wait Until Element Is Visible           xpath=//h1[text() = "${FIRST NAME} ${LAST NAME}"]
    Wait Until Element Is Visible           xpath=//p[text() = "Username: ${USERNAME}"]
    Wait Until Element Is Visible           xpath=//p[text() = "Email: ${EMAIL}"]

Change Email
    Click Element                           xpath=//a[text() = "Update"]
    Wait Until Element Is Visible           xpath=//h1[contains(text(), "Change user")]
    Set Suite Variable                      ${EMAIL}        robot2@test.com
    Input Text                              id:email        ${EMAIL}
    Input Password  id:password             ${PASSWORD}
    Click Element                           xpath=//button[text() = "Update"]
    Wait Until Element Is Visible           xpath=//p[text() = "Email: ${EMAIL}"]

Change Password
    Click Element                           xpath=//a[text() = "Change password"]
    Wait Until Element Is Visible           xpath=//h1[text() = "Change password"]
    Input Password  id:old_password         ${PASSWORD}
    Set Suite Variable                      ${PASSWORD}     Test!user2
    Input Password  id:new_password         ${PASSWORD}
    Click Element                           xpath=//button[text() = "Change"]
    Wait Until Element Is Visible           xpath=//p[text() = "Email: ${EMAIL}"]

Logout Created User
    Wait Until Element Is Visible           xpath=//a[text() = "Logout"]
    Click Element                           xpath=//a[text() = "Logout"]
    Wait Until Element Is Visible           xpath=//a[text() = "Login"]

Check New Password And Email
    keywords.Navigate To Login Page
    Input Text      id:identity             ${EMAIL}
    Input Password  id:password             ${PASSWORD}
    Click Element   id:login-btn
    Wait Until Element Is Visible           xpath=//a[text() = "Profile"]

Request Farm Admin Status
    keywords.Navigate To User Profile
    Click Element                           id:request-adm-btn
    Wait Until Element Is Visible           xpath=//p[text() = "Farm Admin"]

Delete Changed User
    Wait Until Element Is Visible           xpath=//a[text() = "Profile"]
    Click Element                           xpath=//a[text() = "Profile"]
    Wait Until Element Is Visible           xpath=//button[text() = "Delete Account"]
    Input Password  id:password             ${PASSWORD}
    Click Element                           xpath=//button[text() = "Delete Account"]
    Wait Until Element Is Visible           xpath=//a[text() = "Login"]
    Close Browser
