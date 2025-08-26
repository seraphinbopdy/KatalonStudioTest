<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>CreateContratTC001</name>
   <tag></tag>
   <elementGuidId>ba9e1761-c4f3-4efd-b978-d27c12e3441a</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>true</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;userAccount\&quot;: \&quot;${codeCopropriete}\&quot;,\n    \&quot;offerTemplate\&quot;: \&quot;OFFER_EAU_LER\&quot;,\n    \&quot;renewalRule\&quot;: {\n        \&quot;calendarInitialyActiveFor\&quot;: {}\n    },\n    \&quot;subscriptionDate\&quot;: \&quot;{dateSouscription}\&quot;,\n    \&quot;seller\&quot;: \&quot;AGENCE_Toulouse\&quot;,\n    \&quot;customFields\&quot;: {\n        \&quot;customField\&quot;: [\n            {\n                \&quot;code\&quot;: \&quot;CF_ARRONDI_FACTURE\&quot;,\n                \&quot;longValue\&quot;: 4\n            },\n                      {\n                \&quot;code\&quot;: \&quot;CF_MODE_ARRONDI\&quot;,\n                \&quot;stringValue\&quot;: \&quot;À l\u0027inférieur\&quot;\n            },\n            {\n                \&quot;code\&quot;: \&quot;CF_CODE_ANALYTIQUE_AFFAIRE\&quot;,\n                \&quot;stringValue\&quot;: \&quot;999997\&quot;\n            }\n        ]\n    },\n    \&quot;salesPersonName\&quot;: \&quot;\&quot;,\n    \&quot;discountPlanForInstantiation\&quot;: [],\n    \&quot;discountPlanForTermination\&quot;: [],\n    \&quot;priceListCode\&quot;: \&quot;\&quot;,\n    \&quot;paymentMethod\&quot;: null,\n    \&quot;productToInstantiateDto\&quot;: [\n        {\n            \&quot;productCode\&quot;: \&quot;PROD_LACMP\&quot;,\n            \&quot;quantity\&quot;: 30,\n            \&quot;deliveryDate\&quot;: \&quot;${dateDeLivraison}\&quot;,\n            \&quot;attributeInstances\&quot;: [\n                {\n                    \&quot;orderAttributeCode\&quot;: \&quot;ATT_Code_Revision\&quot;,\n                    \&quot;doubleValue\&quot;: \&quot;\&quot;,\n                    \&quot;dateValue\&quot;: \&quot;\&quot;\n                },\n                {\n                    \&quot;orderAttributeCode\&quot;: \&quot;ATT_Technologie\&quot;,\n                    \&quot;stringValue\&quot;: \&quot;RADIO\&quot;,\n                    \&quot;doubleValue\&quot;: \&quot;\&quot;,\n                    \&quot;dateValue\&quot;: \&quot;\&quot;\n                },\n                {\n                    \&quot;orderAttributeCode\&quot;: \&quot;ATT_Prix_Revise\&quot;,\n                    \&quot;stringValue\&quot;: \&quot;\&quot;,\n                    \&quot;dateValue\&quot;: \&quot;\&quot;\n                },\n                {\n                    \&quot;orderAttributeCode\&quot;: \&quot;ATT_Fluide\&quot;,\n                    \&quot;stringValue\&quot;: \&quot;EC\&quot;,\n                    \&quot;doubleValue\&quot;: \&quot;\&quot;,\n                    \&quot;dateValue\&quot;: \&quot;\&quot;\n                },\n                {\n                    \&quot;orderAttributeCode\&quot;: \&quot;ATT_Libelle\&quot;,\n                    \&quot;doubleValue\&quot;: \&quot;\&quot;,\n                    \&quot;dateValue\&quot;: \&quot;\&quot;\n                },\n                {\n                    \&quot;orderAttributeCode\&quot;: \&quot;ATT_Prix_Initial\&quot;,\n                    \&quot;stringValue\&quot;: \&quot;\&quot;,\n                    \&quot;doubleValue\&quot;: 3.95452,\n                    \&quot;dateValue\&quot;: \&quot;\&quot;\n                }\n            ]\n        },\n        {\n            \&quot;productCode\&quot;: \&quot;PROD_PCMP\&quot;,\n            \&quot;quantity\&quot;: 30,\n            \&quot;deliveryDate\&quot;:\&quot;${dateDeLivraison}\&quot;,\n            \&quot;attributeInstances\&quot;: [\n                {\n                    \&quot;orderAttributeCode\&quot;: \&quot;ATT_Libelle\&quot;,\n                    \&quot;doubleValue\&quot;: \&quot;\&quot;,\n                    \&quot;dateValue\&quot;: \&quot;\&quot;\n                },\n                {\n                    \&quot;orderAttributeCode\&quot;: \&quot;ATT_Fluide\&quot;,\n                    \&quot;stringValue\&quot;: \&quot;EC\&quot;,\n                    \&quot;doubleValue\&quot;: \&quot;\&quot;,\n                    \&quot;dateValue\&quot;: \&quot;\&quot;\n                },\n                {\n                    \&quot;orderAttributeCode\&quot;: \&quot;ATT_Prix_Initial\&quot;,\n                    \&quot;stringValue\&quot;: \&quot;\&quot;,\n                    \&quot;doubleValue\&quot;: 2.72548,\n                    \&quot;dateValue\&quot;: \&quot;\&quot;\n                },\n                {\n                    \&quot;orderAttributeCode\&quot;: \&quot;ATT_Prix_Revise\&quot;,\n                    \&quot;stringValue\&quot;: \&quot;\&quot;,\n                    \&quot;dateValue\&quot;: \&quot;\&quot;\n                },\n                {\n                    \&quot;orderAttributeCode\&quot;: \&quot;ATT_Code_Revision\&quot;,\n                    \&quot;doubleValue\&quot;: \&quot;\&quot;,\n                    \&quot;dateValue\&quot;: \&quot;\&quot;\n                },\n                {\n                    \&quot;orderAttributeCode\&quot;: \&quot;ATT_Technologie\&quot;,\n                    \&quot;stringValue\&quot;: \&quot;CLASSIQUE\&quot;,\n                    \&quot;doubleValue\&quot;: \&quot;\&quot;,\n                    \&quot;dateValue\&quot;: \&quot;\&quot;\n                }\n            ]\n        },\n      \t{\n            \&quot;productCode\&quot;: \&quot;PROD_FLCMP\&quot;,\n            \&quot;quantity\&quot;: 1,\n            \&quot;deliveryDate\&quot;: \&quot;${dateDeLivraison}\&quot;,\n            \&quot;attributeInstances\&quot;: [\n                {\n                    \&quot;orderAttributeCode\&quot;: \&quot;ATT_Code_Revision\&quot;,\n                    \&quot;doubleValue\&quot;: \&quot;\&quot;,\n                    \&quot;dateValue\&quot;: \&quot;\&quot;\n                },\n                {\n                    \&quot;orderAttributeCode\&quot;: \&quot;ATT_Nbre_Compteurs\&quot;,\n                    \&quot;doubleValue\&quot;: 5,\n                    \&quot;dateValue\&quot;: \&quot;\&quot;\n                },\n              \t                {\n                    \&quot;orderAttributeCode\&quot;: \&quot;ATT_Nbre_APP\&quot;,\n                    \&quot;doubleValue\&quot;: 10,\n                    \&quot;dateValue\&quot;: \&quot;\&quot;\n                },\n                {\n                    \&quot;orderAttributeCode\&quot;: \&quot;ATT_Technologie\&quot;,\n                    \&quot;stringValue\&quot;: \&quot;RADIO\&quot;,\n                    \&quot;doubleValue\&quot;: \&quot;\&quot;,\n                    \&quot;dateValue\&quot;: \&quot;\&quot;\n                },\n              \t{\n                    \&quot;orderAttributeCode\&quot;: \&quot;ATT_Forfaitaire\&quot;,\n                    \&quot;stringValue\&quot;: true,\n                    \&quot;dateValue\&quot;: \&quot;\&quot;\n                },\n                {\n                    \&quot;orderAttributeCode\&quot;: \&quot;ATT_Prix_Revise\&quot;,\n                    \&quot;stringValue\&quot;: \&quot;\&quot;,\n                    \&quot;dateValue\&quot;: \&quot;\&quot;\n                },\n                {\n                    \&quot;orderAttributeCode\&quot;: \&quot;ATT_Fluide\&quot;,\n                    \&quot;stringValue\&quot;: \&quot;EC\&quot;,\n                    \&quot;doubleValue\&quot;: \&quot;\&quot;,\n                    \&quot;dateValue\&quot;: \&quot;\&quot;\n                },\n                {\n                    \&quot;orderAttributeCode\&quot;: \&quot;ATT_Libelle\&quot;,\n                    \&quot;doubleValue\&quot;: \&quot;\&quot;,\n                    \&quot;dateValue\&quot;: \&quot;\&quot;\n                },\n                {\n                    \&quot;orderAttributeCode\&quot;: \&quot;ATT_Prix_Initial\&quot;,\n                    \&quot;stringValue\&quot;: \&quot;\&quot;,\n                    \&quot;doubleValue\&quot;: 2.895,\n                    \&quot;dateValue\&quot;: \&quot;\&quot;\n                }\n            ]\n        },\n      \n      \t{\n            \&quot;productCode\&quot;: \&quot;PROD_ERCMP\&quot;,\n            \&quot;quantity\&quot;: 5,\n            \&quot;deliveryDate\&quot;: \&quot;${dateDeLivraison}\&quot;,\n            \&quot;attributeInstances\&quot;: [\n                {\n                    \&quot;orderAttributeCode\&quot;: \&quot;ATT_Code_Revision\&quot;,\n                    \&quot;doubleValue\&quot;: \&quot;\&quot;,\n                    \&quot;dateValue\&quot;: \&quot;\&quot;\n                },\n                {\n                    \&quot;orderAttributeCode\&quot;: \&quot;ATT_Technologie\&quot;,\n                    \&quot;stringValue\&quot;: \&quot;RADIO\&quot;,\n                    \&quot;doubleValue\&quot;: \&quot;\&quot;,\n                    \&quot;dateValue\&quot;: \&quot;\&quot;\n                },\n                {\n                    \&quot;orderAttributeCode\&quot;: \&quot;ATT_Prix_Revise\&quot;,\n                    \&quot;stringValue\&quot;: \&quot;\&quot;,\n                    \&quot;dateValue\&quot;: \&quot;\&quot;\n                },\n                {\n                    \&quot;orderAttributeCode\&quot;: \&quot;ATT_Fluide\&quot;,\n                    \&quot;stringValue\&quot;: \&quot;EC\&quot;,\n                    \&quot;doubleValue\&quot;: \&quot;\&quot;,\n                    \&quot;dateValue\&quot;: \&quot;\&quot;\n                },\n                {\n                    \&quot;orderAttributeCode\&quot;: \&quot;ATT_Libelle\&quot;,\n                    \&quot;doubleValue\&quot;: \&quot;\&quot;,\n                    \&quot;dateValue\&quot;: \&quot;\&quot;,\n                  \t\&quot;stringValue\&quot;: \&quot;Just to test this Payload\&quot;\n                },\n                {\n                    \&quot;orderAttributeCode\&quot;: \&quot;ATT_Prix_Initial\&quot;,\n                    \&quot;stringValue\&quot;: \&quot;\&quot;,\n                    \&quot;doubleValue\&quot;: 15.25,\n                    \&quot;dateValue\&quot;: \&quot;\&quot;\n                }\n            ]\n        }\n    ],\n    \&quot;validityDate\&quot;: 1741171420908,\n    \&quot;code\&quot;: \&quot;${codeContrat}\&quot;\n}\n&quot;,
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
      <webElementGuid>b3e9e42c-c799-485a-92a9-0486f7a8cd33</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Basic b3BlbmNlbGwuYWRtaW46b3BlbmNlbGwuYWRtaW4=</value>
      <webElementGuid>807df0e6-7baf-489f-b54e-eed5bbabefa8</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>10.1.1</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${GlobalVariable.BASE_URL_API}/billing/subscription/subscribeAndInstantiateProducts</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>f5f24659-42fd-4191-a6bd-1dd5063bada0</id>
      <masked>false</masked>
      <name>codeContrat</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>4b563390-1954-4502-a2fd-48c80cb4be69</id>
      <masked>false</masked>
      <name>dateDeLivraison</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>a60dc764-a560-4dfa-a9bf-5649ad6a9d35</id>
      <masked>false</masked>
      <name>codeCopropriete</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>7268413f-bf39-4768-83fb-a7d9fb91f8ec</id>
      <masked>false</masked>
      <name>dateSouscription</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
