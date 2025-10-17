*** Settings ***
Resource    keywords.robot

*** Test Cases ***

Register User
    keywords.Open Browser To Index Page
    keywords.Navigate To Register Page
    Input Text      id:firstname    Robot
    Input Text      id:lastname     Tester
    Input Text      id:username     robottester
    Input Text      id:email        robot@test.com
    Input Password  id:password             Test!user1
    Input Password  id:confirm-password     Test!user1
    Click Element   class:registerbtn
    Wait Until Element Is Visible   xpath=//h1[text() = "Hello, farmers"]

Login Created User
    keywords.Navigate To Login Page
    Input Text      id:identity             robottester
    Input Password  id:password             Test!user1
    Click Element   class:loginbtn
    Wait Until Element Is Visible           xpath=//a[text() = "Logout"]

Show Profile
    Click Element                           xpath=//a[text() = "Profile"]
    Wait Until Element Is Visible           xpath=//h1[text() = "Robot Tester"]
    Wait Until Element Is Visible           xpath=//p[text() = "Username: robottester"]
    Wait Until Element Is Visible           xpath=//p[text() = "Email: robot@test.com"]

Change Email
    Click Element                           xpath=//a[text() = "Update"]
    Wait Until Element Is Visible           xpath=//h1[contains(text(), "Change user")]
    Input Text                              id:email        robot2@test.com
    Input Password  id:password             Test!user1
    Click Element                           class:updatebtn
    Wait Until Element Is Visible           xpath=//p[text() = "Email: robot2@test.com"]

Show Farms
    keywords.Navigate To Farm List
    Wait Until Element Is Visible           xpath=//h1[contains(text(), "farm-list")]

Logout Created User
    Wait Until Element Is Visible           xpath=//a[text() = "Logout"]
    Click Element                           xpath=//a[text() = "Logout"]
    Wait Until Element Is Visible           xpath=//a[text() = "Login"]
    Close Browser
