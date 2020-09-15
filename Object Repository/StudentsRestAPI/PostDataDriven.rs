<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>PostDataDriven</name>
   <tag></tag>
   <elementGuidId>3bca8ac6-5c06-4100-a491-ea90c582ac8a</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;first_name\&quot;: \&quot;${firstName}\&quot;,\n  \&quot;middle_name\&quot;: \&quot;${middleName}\&quot;,\n  \&quot;last_name\&quot;: \&quot;${lastName}\&quot;,\n  \&quot;date_of_birth\&quot;: \&quot;${dateOfBirth}\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>http://thetestingworldapi.com/api/studentsDetails</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>'Test'</defaultValue>
      <description></description>
      <id>abee8b60-3c05-4c70-b04a-c8635ae625dd</id>
      <masked>false</masked>
      <name>firstName</name>
   </variables>
   <variables>
      <defaultValue>'Automation'</defaultValue>
      <description></description>
      <id>1e96c4ab-2ea7-44b6-a6af-a4d457c077fa</id>
      <masked>false</masked>
      <name>middleName</name>
   </variables>
   <variables>
      <defaultValue>'World'</defaultValue>
      <description></description>
      <id>b61d796a-f3e1-469b-a265-c0d887d90b7c</id>
      <masked>false</masked>
      <name>lastName</name>
   </variables>
   <variables>
      <defaultValue>'9/14/2020\r\n'</defaultValue>
      <description></description>
      <id>a2ca1214-f1ad-4a77-af11-60e84e65a0cd</id>
      <masked>false</masked>
      <name>dateOfBirth</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()
WS.verifyElementPropertyValue(response, 'first_name', &quot;Test&quot;)</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
