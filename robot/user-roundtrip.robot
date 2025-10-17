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

Show Farms
    keywords.Navigate To Farm List
    Wait Until Element Is Visible           xpath=//h1[contains(text(), "farm-list")]

Logout Created User
    Wait Until Element Is Visible           xpath=//a[text() = "Logout"]
    Click Element                           xpath=//a[text() = "Logout"]
    Wait Until Element Is Visible           xpath=//a[text() = "Login"]
    Close Browser
