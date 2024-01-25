<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Token</name>
   <tag></tag>
   <elementGuidId>390242dd-f459-417f-9476-0c1df5155be6</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;userId\&quot;: \&quot;${user}\&quot;,\n    \&quot;password\&quot;: \&quot;${password}\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>text/plain</value>
      <webElementGuid>14bfdfc7-6def-4954-8634-f503e8479100</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.2.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://fifada-qa-lb01.fifgroup.co.id/backend/user/login</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>'fiona'</defaultValue>
      <description></description>
      <id>fb86c438-4d05-43d5-bd56-0976e020d904</id>
      <masked>false</masked>
      <name>user</name>
   </variables>
   <variables>
      <defaultValue>'zQe9Rk4qqvzs3Xyz'</defaultValue>
      <description></description>
      <id>925ce9d9-f43e-4a0b-b595-c61a7a9f53d7</id>
      <masked>false</masked>
      <name>password</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>178a5c06-0723-45d0-ab83-5038d9d9e6a0</id>
      <masked>false</masked>
      <name>variable</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import java.nio.file.WatchService

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

GlobalVariable.Get_Token = WS.getElementPropertyValue(response, 'result.accessToken')

System.out.println(GlobalVariable.Get_Token) 

//WS.verifyElementPropertyValue(response, 'result.accessToken', &quot;eyJhbGciOiJIUzUxMiJ9.eyJzdWIiOiJ7XCJ1c2VySWRcIjpcImZpb25hXCIsXCJ0eXBlXCI6XCJCQUNLRU5EQURNSU5cIixcImJhY2tlbmRVc2VySWRcIjpcImZpb25hXCJ9IiwianRpIjoiM2NhNmFlNTgtMDcxZS00N2YwLTliMTYtMjUxOGJkOTIwYmRkIiwiaWF0IjoxNzA2MDcxNDIxLCJleHAiOjE3MDYwNzUwMjF9.TH_ht-kIIfyF3lU4BknJGox8KgTGNuBgK87YyLsTwKeWUffRrpQuLrvVuS7CYXn9S1HT_kkxRTydNx-vfwQVpQ&quot;)

</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
